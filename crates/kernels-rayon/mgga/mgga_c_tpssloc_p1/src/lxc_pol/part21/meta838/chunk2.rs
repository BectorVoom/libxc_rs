//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2992/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2992(t17171: f64, t2970: f64, t973: f64, t17167: f64, t10390: f64, t10413: f64, t14189: f64, t14213: f64, t17923: f64, t18025: f64, t2979: f64, t3071: f64, t43200: f64, t43214: f64, t43219: f64, t43221: f64, t43361: f64, t4644: f64, t48477: f64, t50183: f64, t50189: f64, t50229: f64, t5873: f64, t59755: f64, t59763: f64, t977: f64) -> f64 {
    let t62631 = t973 * t2970 * t17171;
    let t62640 = t973 * t2970 * t17167;
    let t62648 = -t50183 / 1728.0_f64 - t50189 / 216.0_f64 - t43361 * t3071 * t5873 * t14213 / 384.0_f64 - t10413 * t3071 * t48477 * t17923 / 1152.0_f64 + 5.0_f64 / 2592.0_f64 * t4644 * t14189 - t43200 / 10368.0_f64 - t62631 / 108.0_f64 + t973 * t977 * t59763 / 48.0_f64 + t973 * t2979 * t59755 / 6.0_f64 + t62640 / 72.0_f64 + t43214 / 1944.0_f64 + t43219 / 5184.0_f64 + t43221 / 1296.0_f64 - t10390 * t18025 / 576.0_f64 - t50229 / 216.0_f64;
    t62648
}
