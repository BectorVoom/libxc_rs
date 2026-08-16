//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1419/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1419<F: Float>(t34695: F, t34701: F, t37032: F, t37033: F, t37034: F, t37035: F, t37036: F, t37037: F, t37038: F, t37040: F, t37042: F, t34742: F, t37058: F, t37059: F, t37060: F, t37061: F, t37062: F, t37063: F, t37064: F, t37065: F, t37066: F, t37067: F) -> (F, F) {
    let t38603 = t37032 - t37033 + t37034 - t37035 - t37036 - t37037 + t37038 + F::cast_from(0.8839704917038230932e-7_f64) * t34695 - t37040 + F::cast_from(0.6629778687778673199e-7_f64) * t34701 - t37042;
    let t38609 = -F::cast_from(0.98332751566569010432e-7_f64) * t34742 - t37058 - t37059 - t37060 + t37061 + t37062 - t37063 + t37064 - t37065 - t37066 - t37067;
    (t38603, t38609)
}
