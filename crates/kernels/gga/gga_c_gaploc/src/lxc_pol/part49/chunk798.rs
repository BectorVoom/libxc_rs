//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 798/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk798<F: Float>(t13066: F, t13070: F, t13074: F, t13079: F, t13114: F, t13115: F, t13116: F, t13120: F, t13890: F, t13893: F, t13895: F, t13898: F, t13899: F, t13901: F) -> F {
    let t13903 = -F::new(0.19171462976960374838e0) * t13066 - t13890 - F::new(0.14896037479937677779e-1) * t13893 + F::new(0.14896037479937677779e-1) * t13895 + F::new(0.19171462976960374838e0) * t13070 - t13074 + t13079 - t13898 - t13114 + t13115 + t13116 + t13899 + t13120 + F::new(0.71500979903700853338e0) * t13901;
    t13903
}
