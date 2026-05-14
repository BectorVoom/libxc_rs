//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1182/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1182<F: Float>(t10930: F, t10931: F, t32897: F, t8556: F, t9823: F, t7336: F, t8775: F, t10893: F, t28937: F, t28941: F, t28944: F, t28946: F, t33820: F, t33824: F, t33826: F, t33829: F, t33832: F, t33835: F, t33838: F, t33841: F, t5694: F) -> (F,) {
    let t33844 = 0.55213813373645879534e2 * t10930 * t10931 * t32897;
    let t33846 = 0.47667319935800568892e0 * t9823 * t8556;
    let t33848 = 0.2780593662921699852e0 * t8775 * t7336;
    let t33849 = -t33820 + 0.92686455430723328401e-1 * t10893 * t5694 + t33824 - t28937 - t28941 + t28944 + t28946 + t33826 - t33829 + t33832 - t33835 - t33838 - t33841 + t33844 + t33846 - t33848;
    (t33849,)
}
