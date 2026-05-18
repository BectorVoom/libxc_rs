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
    let t27757 = -t23577 / F::new(8.0) - t555 * t2987 * t7920 * t3 / F::new(8.0) - t23588 / F::new(96.0) - t23591 / F::new(24.0) - F::new(41.0) / F::new(144.0) * t23625 - F::new(3.0) / F::new(64.0) * t1867 * t3919 - F::new(3.0) / F::new(32.0) * t547 * t10083 + t27719 / F::new(96.0) - t27721 / F::new(32.0) - t27723 / F::new(32.0) - t27725 / F::new(32.0) + t7842 * t2972 * t27728 / F::new(8.0) + t7842 * t2972 * t27732 / F::new(16.0) + F::new(7.0) / F::new(18.0) * t23667 * t7868 * t27649 - t27741 / F::new(36.0) - t2970 * t2971 * t3112 * t2974 / F::new(12.0) - t2970 * t9829 * t7857 / F::new(12.0) - t2970 * t9829 * t7861 / F::new(24.0) - F::new(7.0) / F::new(72.0) * t7866 * t27753 * t7843;
    t27757
}
