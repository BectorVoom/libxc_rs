//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1786/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1786<F: Float>(t1012: F, t1222: F, t1225: F, t1782: F, t21213: F, t21306: F, t24736: F, t24821: F, t24827: F, t24831: F, t24836: F, t3699: F, t44348: F, t44919: F, t5373: F, t57707: F, t6653: F, t83962: F, t87107: F, t87126: F, t87145: F) -> F {
    let t91119 = -F::cast_from(0.25724410870841842184e-2_f64) * t21306 * t24736 + t1222 * t1012 * t44348 * t87145 / F::new(6.0) + F::new(28.0) / F::new(243.0) * t5373 * t24827 + F::new(22.0) / F::new(81.0) * t21213 * t6653 - F::new(8.0) / F::new(27.0) * t5373 * t24831 + F::cast_from(0.27439371595564631662e-1_f64) * t57707 * t24836 + F::new(2.0) / F::new(9.0) * t5373 * t24821 - t1222 * t1012 * t1225 * t87126 / F::new(288.0) - t1222 * t1012 * t44919 * t87145 / F::new(12.0) + t1222 * t1012 * t3699 * t87107 / F::new(72.0) + F::new(154.0) / F::new(243.0) * t83962 * t1782;
    t91119
}
