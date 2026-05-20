//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3751/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3751<F: Float>(t1222: F, t17240: F, t20306: F, t12772: F, t21156: F, t3625: F, t12866: F, t17456: F, t17639: F, t17645: F, t17661: F, t44823: F, t44829: F, t44838: F, t44884: F, t5308: F, t5312: F, t59162: F, t68269: F, t68273: F, t68317: F, t68328: F) -> F {
    let t71377 = t1222 * t17240 * t20306;
    let t71400 = t3625 * t12772 * t21156;
    let t71406 = -t71377 / F::new(72.0) - t1222 * t5308 * t68273 / F::new(72.0) - t1222 * t5308 * t68317 / F::new(48.0) - t1222 * t5308 * t68269 / F::new(12.0) + F::cast_from(0.28582678745379824648e-3_f64) * t12866 * t17661 * t17639 + F::cast_from(0.57165357490759649296e-3_f64) * t12866 * t17661 * t17645 + t1222 * t5312 * t68328 / F::new(6.0) - F::cast_from(0.17149607247227894789e-2_f64) * t59162 * t17456 - F::cast_from(0.19055119163586549765e-3_f64) * t71400 + F::cast_from(0.47637797908966374413e-4_f64) * t44823 - F::cast_from(0.1270341277572436651e-3_f64) * t44829 - F::cast_from(0.95275595817932748826e-4_f64) * t44838 + F::cast_from(0.95275595817932748826e-4_f64) * t44884;
    t71406
}
