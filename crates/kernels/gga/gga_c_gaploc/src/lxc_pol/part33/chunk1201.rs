//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1201/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1201<F: Float>(t31783: F, t31786: F, t31788: F, t31790: F, t31792: F, t31796: F, t31799: F, t31805: F, t31811: F, t31825: F, t31835: F, t31838: F, t31840: F, t31842: F, t31846: F, t31849: F, t31853: F, t31856: F, t31860: F, t31863: F, t31865: F, t31869: F, t31879: F, t31881: F, t31883: F, t31886: F, t31890: F, t31894: F, t31898: F) -> (F, F) {
    let t38369 = -t31783 + t31786 - t31788 - t31790 - t31792 - t31796 + t31799 + t31805 + t31811 - t31825 + t31835 + t31838 - t31840 - t31842 + t31846;
    let t38373 = -t31849 - t31853 - t31856 + t31860 + t31863 + t31865 + t31869 - t31879 + t31881 - t31883 + t31886 - t31890 - t31894 - t31898;
    (t38369, t38373)
}
