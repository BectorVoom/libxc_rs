//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1276/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1276<F: Float>(t28936: F, t28940: F, t28944: F, t28946: F, t33820: F, t33824: F, t33826: F, t33829: F, t33832: F, t33835: F, t33838: F, t33841: F, t33844: F, t33846: F, t33848: F, t33851: F) -> (F,) {
    let t39299 = -t33820 + t33824 - 0.76685851907841499354e0 * t28936 - 0.38342925953920749677e0 * t28940 + t28944 + t28946 + t33826 - t33829 + t33832 - t33835 - t33838 - t33841 + t33844 + t33846 - t33848 + t33851;
    (t39299,)
}
