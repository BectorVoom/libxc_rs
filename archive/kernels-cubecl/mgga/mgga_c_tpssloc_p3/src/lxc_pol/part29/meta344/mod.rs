//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1407;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1408;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta344<F: Float>(t3732: F, t782: F, t3736: F, t1365: F, t154: F, t205: F, t116: F, t547: F, t1307: F, t212: F, t2586: F, t535: F, t9534: F, t9538: F, t3792: F, t3850: F, t1337: F, t550: F, t1338: F, t3879: F, t3773: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12211, t12212, t12214, t12215, t12225, t12228, t12236) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1407::<F>(t3732, t782, t3736, t1365, t154, t205, t116, t547, t1307, t212, t2586, t535, t9534, t9538);
        let (t12240, t12248, t12250, t12259, t12267) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1408::<F>(t3792, t3850, t1337, t550, t1338, t3879, t3773, t68);
    (t12211, t12212, t12214, t12215, t12225, t12228, t12236, t12240, t12248, t12250, t12259, t12267)
}
