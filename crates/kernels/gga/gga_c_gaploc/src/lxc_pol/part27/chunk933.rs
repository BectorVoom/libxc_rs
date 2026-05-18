//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 933/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk933<F: Float>(t3347: F, t6313: F, t3344: F, t484: F, t874: F, t986: F) -> (F, F, F) {
    let t10238 = F::new(0.1138200265427045984e0) * t6313 * t3347;
    let t10239 = t484 * t3344;
    let t10240 = F::new(0.15808337019820083111e-2) * t10239;
    let t10241 = t874 * t986;
    (t10238, t10240, t10241)
}
