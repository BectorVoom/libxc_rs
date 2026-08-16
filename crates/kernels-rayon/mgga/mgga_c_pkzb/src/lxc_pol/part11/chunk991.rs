//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 991/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk991(t10859: f64, t703: f64, t10841: f64, t5873: f64, t10772: f64, t10779: f64, t10782: f64, t10785: f64, t10786: f64, t10789: f64, t10792: f64, t10795: f64, t10830: f64, t10834: f64, t10842: f64, t1096: f64, t1916: f64, t1938: f64, t1955: f64, t1977: f64, t2796: f64, t3578: f64, t3581: f64, t5830: f64, t5845: f64, t5871: f64, t695: f64, t714: f64, t7324: f64, t9518: f64) -> (f64, f64, f64) {
    let t10860 = t10859 * t703;
    let t10863 = t10841 * t5873;
    let t10866 = -0.19751673498613801407e-1_f64 * t10772 + t10779 + t10782 - t10785 - 0.35089341735807877242e1_f64 * t1955 * t10786 + 0.51947577317044391277e2_f64 * t1977 * t10789 - 6.0_f64 * t1916 * t10792 + 0.96491876992155210402e2_f64 * t1938 * t10795 + 3.0_f64 * t9518 * t1096 + 0.5848223622634646207e0_f64 * t714 * t10830 + 0.10254018858216406658e4_f64 * t5845 * t10834 + 3.0_f64 * t2796 * t3578 + 0.96491876992155210402e2_f64 * t7324 * t3581 - 0.19298375398431042081e3_f64 * t5830 * t10842 + 1.0_f64 * t695 * t10860 + 0.2069040516770936012e4_f64 * t5871 * t10863;
    (t10860, t10863, t10866)
}
