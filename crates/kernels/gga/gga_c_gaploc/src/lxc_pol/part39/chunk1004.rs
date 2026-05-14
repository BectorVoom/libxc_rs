//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1004/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1004<F: Float>(t447: F, t46849: F, t204: F, t2476: F, t40219: F, t41909: F, t41915: F, t41919: F, t41922: F, t41927: F, t41930: F, t41933: F, t41935: F, t47949: F, t47951: F, t40225: F) -> (F, F, F) {
    let t47953 = t46849 * t447;
    let t47955 = t2476 * t204 * t47953;
    let t47960 = -0.19171462976960374838e0 * t47949 + 0.35750489951850426669e0 * t47951 + 0.46011511144704899612e1 * t47955 + 0.14896037479937677779e-1 * t41909 + t41915 + t41919 + t41922 + 0.76685851907841499354e0 * t40219 - t41927 - t41930 + t41933 + 0.15337170381568299871e2 * t41935;
    let t47963 = 0.15337170381568299871e1 * t40225;
    (t47953, t47960, t47963)
}
