//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 817/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk817<F: Float>(t1939: F, t247: F, t5831: F, t1947: F, t713: F, t1911: F, t1933: F, t1941: F, t1980: F, t248: F, t5583: F, t5587: F, t5751: F, t5763: F, t5816: F, t5820: F, t5825: F, t5830: F, t5832: F, t5835: F, t5838: F, t5839: F, t5842: F, t5845: F, t5846: F, t5866: F, t5871: F, t695: F, t704: F, t714: F, t723: F) -> (F, F, F, F) {
    let t5873 = F::new(1.0) / t1939 / t247;
    let t5874 = t5831 * t5873;
    let t5877 = t1947 * t713;
    let t5880 = -F::new(0.310907e-1) * t5816 * t248 + t5583 - t5587 - F::cast_from(0.19751673498613801407e-1_f64) * t5763 + t5751 + F::new(3.0) * t5820 * t704 + F::new(3.0) * t1911 * t1933 + F::cast_from(0.96491876992155210402e2_f64) * t5825 * t1941 - F::cast_from(0.19298375398431042081e3_f64) * t5830 * t5832 + F::cast_from(0.51947577317044391276e2_f64) * t5835 * t1980 - F::cast_from(0.10389515463408878255e3_f64) * t5838 * t5839 + F::cast_from(0.5848223622634646207e0_f64) * t714 * t5842 + F::cast_from(0.10254018858216406658e4_f64) * t5845 * t5846 + F::new(1.0) * t695 * t5866 + F::cast_from(0.2069040516770936012e4_f64) * t5871 * t5874 + F::cast_from(0.17544670867903938621e1_f64) * t5877 * t723;
    (t5873, t5874, t5877, t5880)
}
