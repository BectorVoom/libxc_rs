//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2832/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2832(t10900: f64, t14586: f64, t14785: f64, t14791: f64, t1544: f64, t1558: f64, t18393: f64, t23160: f64, t23262: f64, t2730: f64, t2745: f64, t36833: f64, t40507: f64, t40518: f64, t40535: f64, t40868: f64, t4343: f64, t4362: f64, t4366: f64, t50532: f64, t50582: f64, t50605: f64, t5984: f64, t5988: f64, t6035: f64, t61749: f64, t61756: f64, t61774: f64, t61776: f64, t61797: f64, t61817: f64, t76474: f64, t775: f64, t800: f64) -> f64 {
    let t76557 = -t50532 + 0.60023625365297631762e-2_f64 * t61774 + 0.30011812682648815881e-2_f64 * t61776 + t40507 - 0.45738002528356795401e-4_f64 * t40518 + 0.32528867398167352889e-3_f64 * t40535 + 3.0_f64 / 16.0_f64 * t2730 * t800 * t18393 * t1544 + 3.0_f64 / 16.0_f64 * t2730 * t800 * t5984 * t4343 + 5.0_f64 / 4.0_f64 * t40868 * t800 * t23262 * t775 - 3.0_f64 / 4.0_f64 * t10900 * t800 * t5988 * t4343 + 0.76230004213927992336e-5_f64 * t61797 + t50582 + 0.25724410870841842184e-1_f64 * t4362 * t14785 * t76474 * t4366 - 0.51448821741683684368e-2_f64 * t4362 * t14791 * t23160 * t1558 * t775 + 0.25724410870841842183e-2_f64 * t2745 * t14791 * t61756 * t6035 + 0.38586616306262763276e-2_f64 * t4362 * t36833 * t14586 * t61749 - 0.12004725073059526352e-1_f64 * t61817 - t50605;
    t76557
}
