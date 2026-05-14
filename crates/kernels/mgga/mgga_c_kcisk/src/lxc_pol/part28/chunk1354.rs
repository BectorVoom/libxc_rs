//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1354/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1354<F: Float>(t116922: F, t2364: F, t34076: F, t112212: F, t116942: F, t121109: F, t121116: F, t121124: F, t121127: F, t121133: F, t121140: F, t17337: F, t23829: F, t32942: F, t32990: F, t33031: F, t33056: F, t33059: F, t34078: F, t34122: F, t34148: F, t35230: F, t93968: F, t9667: F) -> (F, F) {
    let t121144 = t116922 * t2364 * t34076;
    let t121152 = 0.11054629629629629629e-2 * t121109 + 0.69444444444444444446e-2 * t32942 * t35230 + 0.69444444444444444446e-2 * t32990 * t35230 - 0.34722222222222222223e-2 * t121116 * t9667 - 0.20833333333333333334e-1 * t34122 * t34148 - 0.41666666666666666668e-1 * t34122 * t34078 + 0.22109259259259259259e-2 * t121124 - 0.16581944444444444444e-2 * t121127 + 0.92592592592592592594e-2 * t33031 * t116942 * t23829 - 0.69444444444444444446e-2 * t33031 * t121133 - 0.26805555555555555556e-2 * t33056 * t121133 + 0.35740740740740740741e-2 * t33056 * t121140 - 0.80416666666666666668e-2 * t33056 * t121144 + 0.7716049382716049383e-3 * t112212 - 0.13888888888888888889e-1 * t33031 * t17337 * t33059 * t93968;
    (t121144, t121152)
}
