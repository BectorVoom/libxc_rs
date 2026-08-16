//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3526/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3526(t4866: f64, t4900: f64, t1065: f64, t372: f64, t6299: f64, t3105: f64, t6317: f64, t1068: f64, t15689: f64, t15692: f64, t15907: f64, t16067: f64, t16089: f64, t16128: f64, t16226: f64, t16229: f64, t19705: f64, t19819: f64, t19831: f64, t247: f64, t3092: f64, t3116: f64, t3117: f64, t43297: f64, t4772: f64, t4837: f64, t54599: f64, t54899: f64, t606: f64, t64912: f64, t66752: f64, t66758: f64, t66763: f64, t66766: f64) -> (f64, f64, f64) {
    let t66771 = t4900 * t4866;
    let t66777 = t372 * t1065 * t6299;
    let t66784 = t6317 * t3105;
    let t66793 = 0.11433071498151929859e-2_f64 * t66752 + 0.85748036236139473944e-3_f64 * t4837 * t247 * t3116 * t64912 - 0.57165357490759649296e-3_f64 * t66758 - 0.45732285992607719436e-2_f64 * t43297 * t19831 - 0.6351706387862183255e-4_f64 * t66763 - 0.95275595817932748826e-3_f64 * t66766 * t16128 - 0.25724410870841842184e-2_f64 * t54899 * t19819 + 0.85748036236139473944e-3_f64 * t16067 * t3117 * t15907 * t66771 + 0.57165357490759649296e-3_f64 * t16226 * t66777 * t16229 - 0.28582678745379824648e-3_f64 * t15689 * t66777 * t15692 - 0.15244095330869239812e-2_f64 * t66784 * t1068 + 0.1270341277572436651e-3_f64 * t54599 + 0.11433071498151929859e-2_f64 * t16089 * t3092 * t19705 * t606 * t4772;
    (t66771, t66777, t66793)
}
