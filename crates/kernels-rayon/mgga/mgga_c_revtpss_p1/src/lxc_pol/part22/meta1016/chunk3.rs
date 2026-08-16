//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3511/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3511(t3151: f64, t6258: f64, t11922: f64, t19744: f64, t3115: f64, t20104: f64, t11703: f64, t11875: f64, t13396: f64, t15139: f64, t15153: f64, t15950: f64, t16095: f64, t19750: f64, t19754: f64, t2852: f64, t3117: f64, t3162: f64, t4181: f64, t42410: f64, t42656: f64, t4573: f64, t4772: f64, t53654: f64, t53657: f64, t54099: f64, t54118: f64, t54122: f64, t55011: f64) -> (f64, f64) {
    let t66341 = t6258 * t3151;
    let t66355 = t3115 * t11922 * t19744;
    let t66362 = t3115 * t11922 * t20104;
    let t66373 = 0.25724410870841842184e-2_f64 * t53654 * t19750 - 0.25724410870841842184e-2_f64 * t53657 * t19754 + 0.21437009059034868486e-3_f64 * t11875 * t3117 * t66341 * t3162 - 0.57165357490759649297e-2_f64 * t55011 * t11703 * t15153 * t13396 - 0.95275595817932748828e-3_f64 * t16095 * t11703 * t4573 * t15950 - 0.28582678745379824648e-3_f64 * t66355 + 0.2540682555144873302e-2_f64 * t55011 * t42410 * t15139 * t13396 - 0.28582678745379824648e-3_f64 * t66362 - 0.95275595817932748826e-3_f64 * t16095 * t11703 * t4772 * t2852 * t4181 - 0.3811023832717309953e-3_f64 * t54099 + 0.5081365110289746604e-3_f64 * t42656 + 5.0_f64 / 1944.0_f64 * t54118 + t54122 / 162.0_f64;
    (t66341, t66373)
}
