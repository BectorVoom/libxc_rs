//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 991/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk991<F: Float>(t10859: F, t703: F, t10841: F, t5873: F, t10772: F, t10779: F, t10782: F, t10785: F, t10786: F, t10789: F, t10792: F, t10795: F, t10830: F, t10834: F, t10842: F, t1096: F, t1916: F, t1938: F, t1955: F, t1977: F, t2796: F, t3578: F, t3581: F, t5830: F, t5845: F, t5871: F, t695: F, t714: F, t7324: F, t9518: F) -> (F, F, F) {
    let t10860 = t10859 * t703;
    let t10863 = t10841 * t5873;
    let t10866 = -F::new(0.19751673498613801407e-1) * t10772 + t10779 + t10782 - t10785 - F::new(0.35089341735807877242e1) * t1955 * t10786 + F::new(0.51947577317044391277e2) * t1977 * t10789 - F::new(6.0) * t1916 * t10792 + F::new(0.96491876992155210402e2) * t1938 * t10795 + F::new(3.0) * t9518 * t1096 + F::new(0.5848223622634646207e0) * t714 * t10830 + F::new(0.10254018858216406658e4) * t5845 * t10834 + F::new(3.0) * t2796 * t3578 + F::new(0.96491876992155210402e2) * t7324 * t3581 - F::new(0.19298375398431042081e3) * t5830 * t10842 + F::new(1.0) * t695 * t10860 + F::new(0.2069040516770936012e4) * t5871 * t10863;
    (t10860, t10863, t10866)
}
