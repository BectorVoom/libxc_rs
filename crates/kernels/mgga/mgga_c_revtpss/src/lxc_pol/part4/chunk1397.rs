//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1397/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1397<F: Float>(t13045: F, t3601: F, t17710: F, t3720: F, t1261: F, t12784: F, t17669: F, t17674: F, t17679: F, t17684: F, t17690: F, t17693: F, t17696: F, t17700: F, t17705: F, t17709: F, t3625: F, t3708: F, t5287: F, t5331: F, t5340: F, t5407: F) -> F {
    let t17711 = t13045 * t3601;
    let t17712 = t17710 * t17711;
    let t17713 = t3720 * t17712;
    let t17718 = -F::new(0.28582678745379824648e-3) * t12784 * t5407 - F::new(0.28582678745379824648e-3) * t3625 * t17669 - F::new(0.14291339372689912324e-3) * t3625 * t17674 - F::new(0.28582678745379824648e-3) * t5340 * t17679 + F::new(0.14291339372689912324e-3) * t5331 * t17684 + F::new(0.23818898954483187207e-3) * t3625 * t17690 + F::new(0.47637797908966374414e-3) * t17693 * t17696 + F::new(0.47637797908966374414e-3) * t1261 * t17700 + F::new(0.42874018118069736972e-3) * t5340 * t17705 + F::new(0.12862205435420921092e-2) * t17709 * t17713 + F::new(0.42874018118069736972e-3) * t3708 * t5287;
    t17718
}
