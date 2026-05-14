//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1362/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1362<F: Float>(t25797: F, t2837: F, t6263: F, t783: F, t20928: F, t20932: F, t20944: F, t22850: F, t24145: F, t25372: F, t25764: F, t25769: F, t25773: F, t25780: F, t25781: F, t25793: F, t506: F, t529: F, t6493: F, t6522: F, t7953: F) -> (F,) {
    let t25798 = 0.6112917064160653851e0 * t25797;
    let t25800 = t783 * t2837 * t6263;
    let t25802 = 0.5141876673348786705e0 * t25764 + 0.1047928639570397803e0 * t25769 + 0.59329162131926993722e1 * t20928 + 0.17798748639578098116e2 * t20932 + 0.38415120233790484326e0 * t25773 + 0.31205598264195366828e1 * t6493 * t7953 + t25780 + 0.46098144280548581192e1 * t25781 + 0.32927245914677557994e1 * t22850 * t529 * t506 * t25372 - 0.19756347548806534797e1 * t6522 * t529 * t506 * t24145 + 0.41607464352260489103e1 * t25793 - 0.17465477326173296717e-1 * t20944 + t25798 + 0.73613752582167450608e0 * t25800;
    (t25802,)
}
