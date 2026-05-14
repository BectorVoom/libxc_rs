//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 786/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk786<F: Float>(t475: F, t7995: F, t2343: F, t447: F, t7980: F, t1064: F, t1305: F, t2778: F, t1265: F, t2787: F, t1266: F, t2765: F, t2822: F, t448: F, t1306: F, t999: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7996 = t7995 * t475;
    let t7997 = t2343 * t7996;
    let t8000 = t7980 * t447;
    let t8001 = t1064 * t8000;
    let t8004 = t2778 * t1305;
    let t8005 = t1064 * t8004;
    let t8012 = t2787 * t1265;
    let t8013 = t2343 * t8012;
    let t8016 = t2765 * t1266;
    let t8019 = t2822 * t448;
    let t8022 = t999 * t1306;
    (t7996, t7997, t8000, t8001, t8004, t8005, t8012, t8013, t8016, t8019, t8022)
}
