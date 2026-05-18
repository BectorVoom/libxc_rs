//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 813/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk813<F: Float>(t13005: F, t13006: F, t13013: F, t13016: F, t13021: F, t13023: F, t13025: F, t13042: F, t10283: F, t11157: F, t11160: F, t11166: F, t12381: F, t12891: F, t153: F, t156: F, t168: F, t242: F, t245: F, t4550: F, t4557: F, t4600: F, t5588: F, t5592: F, t5595: F, t7981: F, t8042: F, t8051: F, t8058: F, t8066: F) -> (F, F) {
    let t13045 = t13005 + t13006 + t13013 + t13016 + t13021 + t13023 + t13025 + t13042;
    let t13055 = -F::new(0.25128846160651320563e0) * t11157 + F::new(0.25128846160651320563e0) * t11160 - t4550 + t4557 - t4600 - F::new(0.50257692321302641125e0) * t8042 + F::new(0.42708890021612718669e0) * t153 * t156 * t12381 + t5588 + F::new(0.50257692321302641125e0) * t8051 + F::new(0.39861630686838537423e1) * t7981 - F::new(0.11938374665504764976e-1) * t168 * t245 * t13045 - F::new(0.25128846160651320563e0) * t8058 + t5592 - F::new(0.15917832887339686635e0) * t8066 - t5595 + F::new(0.59691873327523824879e-1) * t11166 - F::new(0.17083556008645087467e1) * t10283 - F::new(0.83762820535504401876e-1) * t12891 * t242;
    (t13045, t13055)
}
