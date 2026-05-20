//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2832/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2832<F: Float>(t10900: F, t14586: F, t14785: F, t14791: F, t1544: F, t1558: F, t18393: F, t23160: F, t23262: F, t2730: F, t2745: F, t36833: F, t40507: F, t40518: F, t40535: F, t40868: F, t4343: F, t4362: F, t4366: F, t50532: F, t50582: F, t50605: F, t5984: F, t5988: F, t6035: F, t61749: F, t61756: F, t61774: F, t61776: F, t61797: F, t61817: F, t76474: F, t775: F, t800: F) -> F {
    let t76557 = -t50532 + F::cast_from(0.60023625365297631762e-2_f64) * t61774 + F::cast_from(0.30011812682648815881e-2_f64) * t61776 + t40507 - F::cast_from(0.45738002528356795401e-4_f64) * t40518 + F::cast_from(0.32528867398167352889e-3_f64) * t40535 + F::new(3.0) / F::new(16.0) * t2730 * t800 * t18393 * t1544 + F::new(3.0) / F::new(16.0) * t2730 * t800 * t5984 * t4343 + F::new(5.0) / F::new(4.0) * t40868 * t800 * t23262 * t775 - F::new(3.0) / F::new(4.0) * t10900 * t800 * t5988 * t4343 + F::cast_from(0.76230004213927992336e-5_f64) * t61797 + t50582 + F::cast_from(0.25724410870841842184e-1_f64) * t4362 * t14785 * t76474 * t4366 - F::cast_from(0.51448821741683684368e-2_f64) * t4362 * t14791 * t23160 * t1558 * t775 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t14791 * t61756 * t6035 + F::cast_from(0.38586616306262763276e-2_f64) * t4362 * t36833 * t14586 * t61749 - F::cast_from(0.12004725073059526352e-1_f64) * t61817 - t50605;
    t76557
}
