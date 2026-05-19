//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1333/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1333<F: Float>(t326: F, t32803: F, t825: F, t10867: F, t2634: F, t4673: F, t32897: F, t7427: F, t7573: F, t10930: F, t10931: F, t8556: F, t9823: F) -> (F, F, F, F, F) {
    let t33835 = F::cast_from(0.18404604457881959845e2_f64) * t825 * t326 * t32803;
    let t33838 = F::cast_from(0.33367123955060398226e1_f64) * t10867 * t4673 * t2634;
    let t33841 = F::cast_from(0.12423108009070322895e3_f64) * t7427 * t7573 * t32897;
    let t33844 = F::cast_from(0.55213813373645879534e2_f64) * t10930 * t10931 * t32897;
    let t33846 = F::cast_from(0.47667319935800568892e0_f64) * t9823 * t8556;
    (t33835, t33838, t33841, t33844, t33846)
}
