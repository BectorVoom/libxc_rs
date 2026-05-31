//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1193/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1193<F: Float>(t2490: F, t2491: F, t7627: F, t160: F, t805: F, t91828: F, t91830: F, t91832: F, t91835: F, t91837: F, t91839: F, t91841: F, t91844: F, t91847: F, t91850: F, t91852: F, t91854: F) -> (F, F, F) {
    let t91857 = t2490 * t7627 * t2491;
    let t91859 = t805 * t160;
    let t91861 = -F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t91828 + t91830 / F::cast_from(8.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t91832 + F::cast_from(15.0_f64) / F::cast_from(4.0_f64) * t91835 + F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t91837 - t91839 / F::cast_from(8.0_f64) - t91841 / F::cast_from(32.0_f64) - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t91844 + F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t91847 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t91850 - F::cast_from(3.0_f64) * t91852 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t91854 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t91857 + F::cast_from(9.0_f64) / F::cast_from(4.0_f64) * t91859;
    (t91857, t91859, t91861)
}
