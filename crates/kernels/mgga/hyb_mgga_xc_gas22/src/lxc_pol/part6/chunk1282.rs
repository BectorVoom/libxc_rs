//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1282/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1282<F: Float>(t1230: F, t7867: F, t10083: F, t1867: F, t23577: F, t23588: F, t23591: F, t23625: F, t23667: F, t27649: F, t27719: F, t27721: F, t27723: F, t27725: F, t27728: F, t27732: F, t27741: F, t2970: F, t2971: F, t2972: F, t2974: F, t2987: F, t3: F, t3112: F, t3919: F, t547: F, t555: F, t7842: F, t7843: F, t7857: F, t7861: F, t7866: F, t7868: F, t7920: F, t9829: F) -> F {
    let t27753 = t7867 * t1230;
    let t27757 = -t23577 / F::cast_from(8.0_f64) - t555 * t2987 * t7920 * t3 / F::cast_from(8.0_f64) - t23588 / F::cast_from(96.0_f64) - t23591 / F::cast_from(24.0_f64) - F::cast_from(41.0_f64) / F::cast_from(144.0_f64) * t23625 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t1867 * t3919 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t547 * t10083 + t27719 / F::cast_from(96.0_f64) - t27721 / F::cast_from(32.0_f64) - t27723 / F::cast_from(32.0_f64) - t27725 / F::cast_from(32.0_f64) + t7842 * t2972 * t27728 / F::cast_from(8.0_f64) + t7842 * t2972 * t27732 / F::cast_from(16.0_f64) + F::cast_from(7.0_f64) / F::cast_from(18.0_f64) * t23667 * t7868 * t27649 - t27741 / F::cast_from(36.0_f64) - t2970 * t2971 * t3112 * t2974 / F::cast_from(12.0_f64) - t2970 * t9829 * t7857 / F::cast_from(12.0_f64) - t2970 * t9829 * t7861 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t7866 * t27753 * t7843;
    t27757
}
