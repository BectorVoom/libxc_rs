//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1203/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1203(t2097: f64, t22857: f64, t102364: f64, t109539: f64, t109553: f64, t109555: f64, t109567: f64, t109579: f64, t109609: f64, t115107: f64, t22975: f64, t23043: f64, t26079: f64, t27837: f64, t28899: f64, t30283: f64, t30309: f64, t4003: f64, t543: f64, t6896: f64, t7295: f64, t7301: f64, t7511: f64, t96374: f64) -> (f64, f64) {
    let t115166 = t2097 * t22857;
    let t115181 = 0.43368140941025997312e-1_f64 * t109539 + t96374 - 0.43368140941025997312e-1_f64 * t109553 + 0.77108554593144223218e-1_f64 * t109555 - 0.86736281882051994623e-1_f64 * t109567 + 0.52041769129231196772e1_f64 * t27837 * t30283 + 0.29272321618148349057e-1_f64 * t109579 + 0.21684070470512998656e-1_f64 * t109609 - 0.65854491829355115987e0_f64 * t7511 * t23043 - 0.68549505033305214441e-2_f64 * t102364 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t115166 * t543 - 0.26020884564615598386e1_f64 * t7295 * t26079 * t115107 * t4003 + 0.26020884564615598386e1_f64 * t27837 * t30309 - 0.39512695097613069591e1_f64 * t7511 * t22975 + 0.39512695097613069591e1_f64 * t28899 * t6896;
    (t115166, t115181)
}
