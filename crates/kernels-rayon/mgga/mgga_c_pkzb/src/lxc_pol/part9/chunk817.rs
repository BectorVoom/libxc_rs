//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 817/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk817(t1939: f64, t247: f64, t5831: f64, t1947: f64, t713: f64, t1911: f64, t1933: f64, t1941: f64, t1980: f64, t248: f64, t5583: f64, t5587: f64, t5751: f64, t5763: f64, t5816: f64, t5820: f64, t5825: f64, t5830: f64, t5832: f64, t5835: f64, t5838: f64, t5839: f64, t5842: f64, t5845: f64, t5846: f64, t5866: f64, t5871: f64, t695: f64, t704: f64, t714: f64, t723: f64) -> (f64, f64, f64, f64) {
    let t5873 = 1.0_f64 / t1939 / t247;
    let t5874 = t5831 * t5873;
    let t5877 = t1947 * t713;
    let t5880 = -0.310907e-1_f64 * t5816 * t248 + t5583 - t5587 - 0.19751673498613801407e-1_f64 * t5763 + t5751 + 3.0_f64 * t5820 * t704 + 3.0_f64 * t1911 * t1933 + 0.96491876992155210402e2_f64 * t5825 * t1941 - 0.19298375398431042081e3_f64 * t5830 * t5832 + 0.51947577317044391276e2_f64 * t5835 * t1980 - 0.10389515463408878255e3_f64 * t5838 * t5839 + 0.5848223622634646207e0_f64 * t714 * t5842 + 0.10254018858216406658e4_f64 * t5845 * t5846 + 1.0_f64 * t695 * t5866 + 0.2069040516770936012e4_f64 * t5871 * t5874 + 0.17544670867903938621e1_f64 * t5877 * t723;
    (t5873, t5874, t5877, t5880)
}
