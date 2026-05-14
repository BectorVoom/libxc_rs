//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1333/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1333<F: Float>(t109287: F, t6333: F, t4226: F, t5886: F, t1517: F, t19861: F, t32255: F, t6313: F, t32266: F, t6318: F, t21098: F, t33652: F, t4185: F, t5606: F, t4182: F, t21071: F) -> (F, F, F, F, F, F, F, F, F) {
    let t113398 = t109287 * t6333;
    let t113400 = t5886 * t4226;
    let t113402 = t19861 * t1517;
    let t113404 = t32255 * t6313;
    let t113406 = t32266 * t6318;
    let t113408 = t33652 * t21098;
    let t113410 = t5606 * t4185;
    let t113412 = t5606 * t4182;
    let t113414 = t33652 * t21071;
    (t113398, t113400, t113402, t113404, t113406, t113408, t113410, t113412, t113414)
}
