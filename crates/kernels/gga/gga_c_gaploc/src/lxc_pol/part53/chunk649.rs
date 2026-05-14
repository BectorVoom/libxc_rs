//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 649/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk649<F: Float>(t12452: F, t12456: F, t12924: F, t12928: F, t12929: F, t12930: F, t12935: F, t12936: F, t12937: F, t12941: F, t13780: F, t13783: F, t12539: F, t12948: F, t12952: F, t12955: F, t12958: F, t13789: F, t13793: F, t13795: F, t13796: F, t13798: F, t13802: F, t13806: F) -> (F, F) {
    let t14463 = t12924 - t12928 - t12929 + t12930 - 0.89376224879626066675e-1 * t12452 + 0.59584149919750711115e-1 * t12456 - 0.38342925953920749676e0 * t13780 + 0.38342925953920749676e0 * t13783 - t12935 + t12936 + t12937 - t12941;
    let t14472 = 0.29792074959875355558e-1 * t13789 - 0.29792074959875355558e-1 * t13793 - t12948 + t13795 - t13796 - 0.76685851907841499353e0 * t12539 + t12952 - t12955 - 0.76685851907841499352e0 * t12958 + 0.71500979903700853338e0 * t13798 - 0.92023022289409799224e1 * t13802 + 0.23005755572352449806e2 * t13806;
    (t14463, t14472)
}
