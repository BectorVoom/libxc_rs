//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1087/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1087(t28280: f64, t28294: f64, t11230: f64, t1282: f64, t1291: f64, t15692: f64, t1872: f64, t27100: f64, t27991: f64, t27993: f64, t27995: f64, t27996: f64, t28004: f64, t28007: f64, t28011: f64, t28072: f64, t28260: f64, t28265: f64, t3664: f64, t5360: f64, t7812: f64, t7823: f64, t8117: f64) -> (f64, f64) {
    let t28295 = t28280 + t28294;
    let t28297 = -6.0_f64 * t11230 * t28260 - t1282 * t28295 - t1291 * t28265 + 2.0_f64 * t15692 * t7812 - t1872 * t27100 - t3664 * t8117 - t5360 * t7823 - t27991 + t27993 - t27995 + t27996 - t28004 - t28007 - t28011 + t28072;
    (t28295, t28297)
}
