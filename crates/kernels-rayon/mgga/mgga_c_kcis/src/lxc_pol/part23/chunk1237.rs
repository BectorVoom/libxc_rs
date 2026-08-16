//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1237/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1237(t28351: f64, t59319: f64, t28357: f64, t4142: f64, t12265: f64, t1464: f64, t28504: f64, t491: f64, t16782: f64, t16892: f64, t27369: f64, t27453: f64, t28348: f64, t28388: f64, t7908: f64, t8155: f64, t94208: f64, t94331: f64, t98159: f64, t98171: f64, t98174: f64, t98179: f64, t98188: f64) -> (f64, f64, f64, f64) {
    let t98190 = t28351 * t59319;
    let t98193 = t4142 * t28357;
    let t98201 = t1464 * t12265 * t491 * t28504;
    let t98203 = -0.55273148148148148147e-3_f64 * t98171 + 0.10203017057291666667e-2_f64 * t27369 * t98174 + 0.55273148148148148147e-3_f64 * t98179 - 0.23168402777777777778e-3_f64 * t94331 * t8155 - 0.18534722222222222222e-2_f64 * t7908 * t16892 * t27453 * t16782 + 0.27636574074074074073e-2_f64 * t98188 - 0.37134344353515625e-4_f64 * t28388 * t98190 - 0.5895802469135802469e-2_f64 * t98193 + 0.18550940104166666667e-3_f64 * t27369 * t98159 - 0.18550940104166666667e-3_f64 * t94208 * t28348 + 0.99491666666666666664e-2_f64 * t98201;
    (t98190, t98193, t98201, t98203)
}
