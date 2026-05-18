//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1342/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1342<F: Float>(t10455: F, t4950: F, t10140: F, t1572: F, t4673: F, t10348: F, t8155: F, t31770: F, t6824: F, t20367: F, t31775: F, t10537: F, t4379: F) -> (F, F, F, F, F, F) {
    let t34900 = F::new(0.95334639871601137784e0) * t4950 * t10455;
    let t34903 = F::new(0.95334639871601137784e0) * t1572 * t4673 * t10140;
    let t34905 = F::new(0.14300195980740170668e1) * t8155 * t10348;
    let t34910 = F::new(0.95334639871601137784e0) * t6824 * t31770;
    let t34912 = F::new(0.47667319935800568892e0) * t20367 * t31775;
    let t34913 = t4379 * t10537;
    (t34900, t34903, t34905, t34910, t34912, t34913)
}
