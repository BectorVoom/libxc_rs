//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1104/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1104<F: Float>(t16686: F, t4153: F, t7923: F, t28351: F, t59319: F, t28357: F, t4142: F, t12265: F, t1464: F, t28504: F, t491: F, t16782: F, t16892: F, t27369: F, t27453: F, t28348: F, t28388: F, t7908: F, t8155: F, t94208: F, t94331: F, t98159: F, t98171: F, t98174: F, t98179: F) -> (F, F, F, F, F) {
    let t98188 = t4153 * t7923 * t16686;
    let t98190 = t28351 * t59319;
    let t98193 = t4142 * t28357;
    let t98201 = t1464 * t12265 * t491 * t28504;
    let t98203 = -0.55273148148148148147e-3 * t98171 + 0.10203017057291666667e-2 * t27369 * t98174 + 0.55273148148148148147e-3 * t98179 - 0.23168402777777777778e-3 * t94331 * t8155 - 0.18534722222222222222e-2 * t7908 * t16892 * t27453 * t16782 + 0.27636574074074074073e-2 * t98188 - 0.37134344353515625e-4 * t28388 * t98190 - 0.5895802469135802469e-2 * t98193 + 0.18550940104166666667e-3 * t27369 * t98159 - 0.18550940104166666667e-3 * t94208 * t28348 + 0.99491666666666666664e-2 * t98201;
    (t98188, t98190, t98193, t98201, t98203)
}
