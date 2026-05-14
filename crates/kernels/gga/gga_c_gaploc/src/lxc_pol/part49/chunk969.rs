//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 969/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk969<F: Float>(t41425: F, t44110: F, t44111: F, t44112: F, t47506: F, t47509: F, t47511: F, t47512: F, t47513: F, t47515: F, t47517: F, t47519: F, t13866: F, t5782: F, t1445: F, t2087: F, t39027: F, t935: F) -> (F, F, F) {
    let t47521 = 0.42603251059911944084e-1 * t47506 + 0.14896037479937677779e-1 * t47509 + t47511 + t47512 - t47513 + 0.10224780254378866581e1 * t41425 + t47515 + t44110 - t44111 + t44112 + 0.29792074959875355558e-1 * t47517 - 0.10725146985555128001e1 * t47519;
    let t47527 = t5782 * t13866;
    let t47531 = t2087 * t1445 * t39027 * t935;
    (t47521, t47527, t47531)
}
