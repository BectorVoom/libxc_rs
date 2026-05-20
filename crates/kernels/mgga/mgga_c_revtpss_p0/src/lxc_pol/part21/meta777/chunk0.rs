//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2768/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2768<F: Float>(t50880: F, t40067: F, t40072: F, t40167: F, t40171: F, t40184: F, t50857: F, t50861: F, t50864: F, t50866: F, t50869: F, t50871: F, t50872: F, t50874: F, t50875: F, t50876: F, t50879: F) -> (F, F) {
    let t50881 = F::new(72.0) * t50880;
    let t50882 = -t50857 + t50861 + t50864 + t50866 + t50869 + t50871 - t50872 + t40067 - t40072 + t50874 + t40167 - t40171 - t50875 + t50876 - t40184 + t50879 + t50881;
    (t50881, t50882)
}
