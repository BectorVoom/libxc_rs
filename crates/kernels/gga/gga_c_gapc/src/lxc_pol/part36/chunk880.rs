//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 880/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk880<F: Float>(t8972: F, t8974: F, t8976: F, t8978: F, t8980: F, t8982: F, t8984: F, t8988: F, t8990: F, t8994: F, t8996: F, t9000: F, t9003: F) -> F {
    let t10662 = F::new(0.30353495895471971564e-6) * t8972 - F::new(0.53968515702149165441e-6) * t8974 + F::new(0.43284943850479925795e-3) * t8976 - F::new(0.43284943850479925795e-3) * t8978 - F::new(0.41223756048076119805e-5) * t8980 + F::new(0.73295838253479341016e-5) * t8982 + F::new(0.24761136101158459626e-5) * t8984 + F::new(0.16009199995585360443e-6) * t8988 + F::new(0.55603792169291016668e-2) * t8990 + F::new(0.18550690221634253912e-3) * t8994 + F::new(0.15458908518028544927e-5) * t8996 - F::new(0.2748593934505475288e-5) * t9000 - F::new(0.2471588561924985691e-3) * t9003;
    t10662
}
