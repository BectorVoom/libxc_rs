//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1419/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1419(t34695: f64, t34701: f64, t37032: f64, t37033: f64, t37034: f64, t37035: f64, t37036: f64, t37037: f64, t37038: f64, t37040: f64, t37042: f64, t34742: f64, t37058: f64, t37059: f64, t37060: f64, t37061: f64, t37062: f64, t37063: f64, t37064: f64, t37065: f64, t37066: f64, t37067: f64) -> (f64, f64) {
    let t38603 = t37032 - t37033 + t37034 - t37035 - t37036 - t37037 + t37038 + 0.8839704917038230932e-7_f64 * t34695 - t37040 + 0.6629778687778673199e-7_f64 * t34701 - t37042;
    let t38609 = -0.98332751566569010432e-7_f64 * t34742 - t37058 - t37059 - t37060 + t37061 + t37062 - t37063 + t37064 - t37065 - t37066 - t37067;
    (t38603, t38609)
}
