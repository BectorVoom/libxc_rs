//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1488;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1489;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta372<F: Float>(t13520: F, t2845: F, t10650: F, t1557: F, t2787: F, t4396: F, t2770: F, t3966: F, t607: F, t2826: F, t136: F, t2250: F, t4337: F, t10216: F, t1409: F, t2244: F, t10304: F, t2775: F, t908: F, t4342: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13522, t13524, t13526, t13528, t13530, t13532) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1488::<F>(t13520, t2845, t10650, t1557, t2787, t4396, t2770, t3966, t607, t2826, t136, t2250, t4337);
        let (t13534, t13537, t13539, t13542, t13544, t13546) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1489::<F>(t13532, t2826, t136, t10216, t1409, t2244, t10304, t2775, t3966, t607, t908, t2250, t4342);
    (t13522, t13524, t13526, t13528, t13530, t13532, t13534, t13537, t13539, t13542, t13544, t13546)
}
