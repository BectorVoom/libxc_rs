//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1280/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1280<F: Float>(t3209: F, t51682: F, t14121: F, t8761: F, t8806: F, t13917: F, t14424: F, t9371: F, t51898: F, t9243: F, t1105: F, t12213: F, t13994: F, t14106: F, t14627: F, t2376: F, t2408: F, t2409: F, t3066: F, t4385: F, t51719: F, t51724: F, t51726: F, t51745: F, t53790: F, t53795: F, t53804: F, t6781: F, t6793: F) -> F {
    let t53806 = t51682 * t3209;
    let t53807 = F::new(7.0) / F::new(24.0) * t53806;
    let t53809 = t14121 * t8761;
    let t53811 = t14121 * t8806;
    let t53816 = t13917 * t14424 * t9371;
    let t53832 = t51898 * t9243;
    let t53834 = -t4385 * t53790 / F::new(48.0) - t6793 * t53795 / F::new(8.0) + t53804 / F::new(768.0) - t53807 + F::new(7.0) / F::new(288.0) * t51719 + t53809 / F::new(16.0) + t53811 / F::new(8.0) - F::new(7.0) / F::new(144.0) * t51724 - F::new(7.0) / F::new(72.0) * t51726 - t53816 / F::new(768.0) + t3066 * t2409 * t12213 * t13994 / F::new(24.0) + t2408 * t2409 * t6781 * t14627 / F::new(24.0) - F::new(7.0) / F::new(72.0) * t51745 + t2408 * t2409 * t2376 * t14106 * t1105 / F::new(48.0) - t53832 / F::new(4.0);
    t53834
}
