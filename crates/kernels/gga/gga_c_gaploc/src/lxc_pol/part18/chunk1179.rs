//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1179/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1179<F: Float>(t11000: F, t783: F, t2714: F, t8634: F, t2718: F, t24817: F, t955: F, t14626: F, t2087: F, t3503: F, t1029: F, t7419: F, t9796: F, t10782: F, t11083: F, t11101: F, t11105: F, t1445: F, t1628: F, t1880: F, t2009: F, t2066: F, t2197: F, t28915: F, t28917: F, t28920: F, t313: F, t314: F, t317: F, t32313: F, t3451: F, t3464: F, t4585: F, t5629: F, t773: F, t780: F, t797: F, t833: F) -> (F,) {
    let t33778 = t11000 * t783;
    let t33786 = 0.71500979903700853338e0 * t2714 * t8634;
    let t33788 = 0.71500979903700853338e0 * t2718 * t8634;
    let t33790 = 0.35750489951850426669e0 * t955 * t24817;
    let t33799 = 0.30674340763136599741e1 * t2087 * t14626 * t3503;
    let t33813 = t9796 * t1029 * t7419;
    let t33814 = 0.76685851907841499352e0 * t33813;
    let t33815 = 0.47667319935800568892e0 * t780 * t33778 + 0.46011511144704899612e1 * t5629 * t1445 * t10782 * t1880 + t33786 + t33788 + t33790 - 0.71500979903700853338e0 * t2066 * t3464 * t2009 - 0.71500979903700853338e0 * t773 * t11083 * t2009 - t33799 + 0.79445533226334281487e-1 * t797 * t4585 * t3451 + 0.61348681526273199482e1 * t833 * t1628 * t11101 + 0.61348681526273199482e1 * t2197 * t11105 + 0.35750489951850426669e0 * t313 * t314 * t32313 * t317 - t28915 - t28917 + t28920 - t33814;
    (t33815,)
}
