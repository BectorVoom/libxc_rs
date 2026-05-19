//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 883/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk883<F: Float>(t8972: F, t8974: F, t8976: F, t8978: F, t8980: F, t8982: F, t8984: F, t8988: F, t8990: F, t8994: F, t8996: F, t9000: F, t9003: F) -> F {
    let t10662 = F::cast_from(0.30353495895471971564e-6_f64) * t8972 - F::cast_from(0.53968515702149165441e-6_f64) * t8974 + F::cast_from(0.43284943850479925795e-3_f64) * t8976 - F::cast_from(0.43284943850479925795e-3_f64) * t8978 - F::cast_from(0.41223756048076119805e-5_f64) * t8980 + F::cast_from(0.73295838253479341016e-5_f64) * t8982 + F::cast_from(0.24761136101158459626e-5_f64) * t8984 + F::cast_from(0.16009199995585360443e-6_f64) * t8988 + F::cast_from(0.55603792169291016668e-2_f64) * t8990 + F::cast_from(0.18550690221634253912e-3_f64) * t8994 + F::cast_from(0.15458908518028544927e-5_f64) * t8996 - F::cast_from(0.2748593934505475288e-5_f64) * t9000 - F::cast_from(0.2471588561924985691e-3_f64) * t9003;
    t10662
}
