//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta143 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk811;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk812;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta143<F: Float>(t1021: F, t248: F, t3132: F, t135: F, t999: F, t973: F, t2250: F, t998: F, t974: F, t2770: F, t2978: F, t2244: F, t2775: F, t976: F, t1005: F, t1036: F, t221: F, t2965: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3134, t3139, t3140, t3142, t3143, t3147) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk811::<F>(t1021, t248, t3132, t135, t999, t973, t2250, t998, t974, t2770, t2978, t2244);
        let (t3148, t3152, t3153, t3156, t3158) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk812::<F>(t3147, t974, t2775, t976, t2244, t1005, t1036, t221, t2965);
    (t3134, t3139, t3140, t3142, t3143, t3147, t3148, t3152, t3153, t3156, t3158)
}
