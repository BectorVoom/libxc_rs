//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1102/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1102<F: Float>(t47371: F, t1044: F, t10691: F, t12596: F, t12814: F, t12869: F, t1620: F, t1621: F, t186: F, t198: F, t25208: F, t2607: F, t2615: F, t30889: F, t31133: F, t31267: F, t3390: F, t3410: F, t3414: F, t3456: F, t3473: F, t3488: F, t3553: F, t40855: F, t4927: F, t5218: F, t561: F, t639: F) -> (F, F) {
    let t47638 = -F::cast_from(12.0_f64) * t47371;
    let t47672 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t561 * t186 * t198 * t47638 - F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t30889 + F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t3488 * t3456 - F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1620 * t1621 * t10691 * t3553 - F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1620 * t1621 * t2607 * t12869 + F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t639 * t1621 * t40855 * t1044 + F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t2615 * t12814 + F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t639 * t4927 * t3473 * t3390 - F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t25208 * t12596 - F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t5218 * t31267 * t3414 - F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t5218 * t31133 * t3410;
    (t47638, t47672)
}
