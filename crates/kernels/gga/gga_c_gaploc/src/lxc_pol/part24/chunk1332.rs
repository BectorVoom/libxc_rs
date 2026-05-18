//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1332/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1332<F: Float>(t33813: F, t10782: F, t11083: F, t11101: F, t11105: F, t1445: F, t1628: F, t1880: F, t2009: F, t2066: F, t2197: F, t28915: F, t28917: F, t28920: F, t313: F, t314: F, t317: F, t32313: F, t33778: F, t33786: F, t33788: F, t33790: F, t33799: F, t3451: F, t3464: F, t4585: F, t5629: F, t773: F, t780: F, t797: F, t833: F) -> F {
    let t33814 = F::new(0.76685851907841499352e0) * t33813;
    let t33815 = F::new(0.47667319935800568892e0) * t780 * t33778 + F::new(0.46011511144704899612e1) * t5629 * t1445 * t10782 * t1880 + t33786 + t33788 + t33790 - F::new(0.71500979903700853338e0) * t2066 * t3464 * t2009 - F::new(0.71500979903700853338e0) * t773 * t11083 * t2009 - t33799 + F::new(0.79445533226334281487e-1) * t797 * t4585 * t3451 + F::new(0.61348681526273199482e1) * t833 * t1628 * t11101 + F::new(0.61348681526273199482e1) * t2197 * t11105 + F::new(0.35750489951850426669e0) * t313 * t314 * t32313 * t317 - t28915 - t28917 + t28920 - t33814;
    t33815
}
