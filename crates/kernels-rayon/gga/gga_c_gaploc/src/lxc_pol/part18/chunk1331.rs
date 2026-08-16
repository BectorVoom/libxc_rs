//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1331/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1331(t33813: f64, t10782: f64, t11083: f64, t11101: f64, t11105: f64, t1445: f64, t1628: f64, t1880: f64, t2009: f64, t2066: f64, t2197: f64, t28915: f64, t28917: f64, t28920: f64, t313: f64, t314: f64, t317: f64, t32313: f64, t33778: f64, t33786: f64, t33788: f64, t33790: f64, t33799: f64, t3451: f64, t3464: f64, t4585: f64, t5629: f64, t773: f64, t780: f64, t797: f64, t833: f64) -> f64 {
    let t33814 = 0.76685851907841499352e0_f64 * t33813;
    let t33815 = 0.47667319935800568892e0_f64 * t780 * t33778 + 0.46011511144704899612e1_f64 * t5629 * t1445 * t10782 * t1880 + t33786 + t33788 + t33790 - 0.71500979903700853338e0_f64 * t2066 * t3464 * t2009 - 0.71500979903700853338e0_f64 * t773 * t11083 * t2009 - t33799 + 0.79445533226334281487e-1_f64 * t797 * t4585 * t3451 + 0.61348681526273199482e1_f64 * t833 * t1628 * t11101 + 0.61348681526273199482e1_f64 * t2197 * t11105 + 0.35750489951850426669e0_f64 * t313 * t314 * t32313 * t317 - t28915 - t28917 + t28920 - t33814;
    t33815
}
