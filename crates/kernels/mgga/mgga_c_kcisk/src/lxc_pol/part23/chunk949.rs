//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 949/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk949<F: Float>(t11250: F, t12845: F, t12847: F, t12855: F, t12857: F, t1421: F, t19146: F, t19150: F, t19152: F, t19156: F, t19160: F, t19163: F, t19223: F, t19227: F, t19230: F, t19235: F, t19237: F, t19241: F, t19246: F, t19251: F, t19255: F, t19258: F, t19262: F, t456: F) -> (F,) {
    let t19265 = 0.1478346675e-2 * t456 * t19146 + t12845 + t19150 + 0.98556445e-3 * t12847 * t19152 - 0.19711289e-2 * t12847 * t19156 + 0.13140859333333333333e-2 * t11250 * t19160 - 0.32852148333333333333e-3 * t19163 - 0.98556445e-3 * t456 * t19223 - t19227 - 0.36958666875e-3 * t1421 * t19230 + 0.13140859333333333334e-2 * t12855 - 0.8760572888888888889e-3 * t12857 + 0.21901432222222222222e-3 * t19235 - 0.8760572888888888889e-3 * t19237 - 0.13140859333333333333e-2 * t1421 * t19241 - 0.1478346675e-2 * t1421 * t19246 - 0.19711289e-2 * t1421 * t19251 + t19255 + 0.19711289e-2 * t1421 * t19258 + 0.98556445e-3 * t1421 * t19262;
    (t19265,)
}
