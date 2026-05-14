//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 774/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk774<F: Float>(t9932: F, t9934: F, t3434: F, t949: F, t2749: F, t3348: F, t3322: F, t9414: F, t9898: F, t9901: F, t9904: F, t9908: F, t9910: F, t9914: F, t9917: F, t9924: F, t9930: F) -> (F,) {
    let t9935 = t9932 * t9934;
    let t9937 = t3434 * t949;
    let t9939 = t3348 * t2749;
    let t9941 = t9414 * t3322;
    let t9943 = 0.12890821708151275006e-8 * t9898 + 0.21135226489492151266e-6 * t9901 + 0.61900849231692170544e-6 * t9904 - 0.42205124476153752644e-7 * t9908 - 0.84410248952307505288e-7 * t9910 - 0.42205124476153752644e-7 * t9914 - 0.50027140879067581468e-8 * t9917 + 0.10005428175813516294e-7 * t9924 - 0.72956247115306889641e-9 * t9930 + 0.24619655944423022376e-7 * t9935 + 0.10821235962619981449e-3 * t9937 + 0.11594181388521408695e-4 * t9939 + 0.40021712703254065174e-7 * t9941;
    (t9943,)
}
