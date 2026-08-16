//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2992/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2992<F: Float>(t17171: F, t2970: F, t973: F, t17167: F, t10390: F, t10413: F, t14189: F, t14213: F, t17923: F, t18025: F, t2979: F, t3071: F, t43200: F, t43214: F, t43219: F, t43221: F, t43361: F, t4644: F, t48477: F, t50183: F, t50189: F, t50229: F, t5873: F, t59755: F, t59763: F, t977: F) -> F {
    let t62631 = t973 * t2970 * t17171;
    let t62640 = t973 * t2970 * t17167;
    let t62648 = -t50183 / F::cast_from(1728.0_f64) - t50189 / F::cast_from(216.0_f64) - t43361 * t3071 * t5873 * t14213 / F::cast_from(384.0_f64) - t10413 * t3071 * t48477 * t17923 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t4644 * t14189 - t43200 / F::cast_from(10368.0_f64) - t62631 / F::cast_from(108.0_f64) + t973 * t977 * t59763 / F::cast_from(48.0_f64) + t973 * t2979 * t59755 / F::cast_from(6.0_f64) + t62640 / F::cast_from(72.0_f64) + t43214 / F::cast_from(1944.0_f64) + t43219 / F::cast_from(5184.0_f64) + t43221 / F::cast_from(1296.0_f64) - t10390 * t18025 / F::cast_from(576.0_f64) - t50229 / F::cast_from(216.0_f64);
    t62648
}
