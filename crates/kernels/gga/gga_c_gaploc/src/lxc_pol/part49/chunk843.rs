//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 843/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk843<F: Float>(t39766: F, t12770: F, t484: F, t10156: F, t2268: F, t6763: F, t10151: F, t2343: F, t6509: F, t10340: F, t6320: F, t41774: F, t41778: F, t12840: F, t6313: F, t12807: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42773 = 0.31616674039640166221e-2 * t39766;
    let t42774 = t484 * t12770;
    let t42778 = t2268 * t10156 * t6763;
    let t42782 = t2268 * t2343 * t10151 * t6509;
    let t42786 = t2268 * t6320 * t10340 * t6509;
    let t42790 = 0.56910013271352299198e-1 * t2268 * t2343 * t41774;
    let t42793 = 0.56910013271352299198e-1 * t2268 * t2343 * t41778;
    let t42795 = 0.1138200265427045984e0 * t6313 * t12840;
    let t42797 = 0.22764005308540919679e0 * t6313 * t12807;
    (t42773, t42774, t42778, t42782, t42786, t42790, t42793, t42795, t42797)
}
