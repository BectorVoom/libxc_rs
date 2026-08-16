//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 699/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk699(t123: f64, t1570: f64, t1581: f64, t1587: f64, t1589: f64, t1614: f64, t1621: f64, t4888: f64, t4892: f64, t49: f64, t4902: f64, t4907: f64, t4912: f64, t4916: f64, t4921: f64, t4922: f64, t4953: f64, t4958: f64, t4961: f64, t4966: f64, t4967: f64, t4979: f64, t4982: f64, t4996: f64, t5005: f64, t5011: f64, t520: f64, t525: f64, t527: f64, t535: f64) -> f64 {
    let t5012 = -0.35089341735807877242e1_f64 * t1614 * t4888 + 0.51947577317044391277e2_f64 * t1621 * t4892 + 0.96491876992155210402e2_f64 * t1587 * t1581 * t1589 * t525 - 6.0_f64 * t1570 * t527 * t1581 + 0.56968947174242584612e-3_f64 * t49 * t4902 * t123 + 6.0_f64 * t1587 * t4907 + 0.10254018858216406658e4_f64 * t4912 * t4916 - 0.10389515463408878255e3_f64 * t4921 * t4922 + 0.5848223622634646207e0_f64 * t535 * t4953 + 0.2069040516770936012e4_f64 * t4958 * t4961 - 0.19298375398431042081e3_f64 * t4966 * t4967 + 1.0_f64 * t520 * t4979 + 0.35089341735807877242e1_f64 * t1621 * t4982 - t4996 - t5005 + t5011;
    t5012
}
