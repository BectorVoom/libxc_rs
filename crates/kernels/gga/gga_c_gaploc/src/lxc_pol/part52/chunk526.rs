//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 526/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk526<F: Float>(t10227: F, t6525: F, t3355: F, t6313: F, t3347: F, t3344: F, t484: F, t874: F, t986: F) -> (F, F, F, F, F, F, F) {
    let t10228 = t6525 * t10227;
    let t10229 = F::new(0.11856252764865062333e-2) * t10228;
    let t10236 = F::new(0.7588001769513639893e-1) * t6313 * t3355;
    let t10238 = F::new(0.1138200265427045984e0) * t6313 * t3347;
    let t10239 = t484 * t3344;
    let t10240 = F::new(0.15808337019820083111e-2) * t10239;
    let t10241 = t874 * t986;
    (t10228, t10229, t10236, t10238, t10239, t10240, t10241)
}
