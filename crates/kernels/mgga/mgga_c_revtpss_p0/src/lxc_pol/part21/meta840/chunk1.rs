//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3150/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3150<F: Float>(t56228: F, t43858: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t56212: F, t56214: F, t56216: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t56248: F, t56252: F, t56256: F) -> F {
    let t58090 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t56228;
    let t58105 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t56212 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t56214 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t56216 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t56221 + F::cast_from(2.0_f64) * t56226 + t58090 - t56230 / F::cast_from(3.0_f64) + t56234 / F::cast_from(3.0_f64) - F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t56236 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t43858 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t43865 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43883 - F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t43888 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43890 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43892 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t43894 - t43896 / F::cast_from(9.0_f64) + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t56248 + F::cast_from(6.0_f64) * t56252 - F::cast_from(4.0_f64) * t56256;
    t58105
}
