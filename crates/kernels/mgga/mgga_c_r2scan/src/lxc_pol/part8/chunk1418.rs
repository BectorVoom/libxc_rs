//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1418/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1418<F: Float>(t10135: F, t1577: F, t1632: F, t551: F, t2122: F, t2573: F, t26037: F, t26039: F, t26042: F, t26053: F, t26060: F, t30496: F, t30498: F, t30500: F, t30535: F, t30559: F, t32485: F, t32679: F, t34390: F, t5109: F, t6583: F, t7337: F) -> (F,) {
    let t34395 = t1577 * t551 * t1632 * t10135;
    let t34410 = -0.1047928639570397803e0 * t34390 - t26037 + t26039 + 0.59329162131926993721e1 * t26042 + t26053 - 0.69345773920434148504e0 * t34395 + 0.26023093918533882311e-2 * t26060 + 0.40752780427737692339e0 * t30496 + 0.87816964854445047169e-1 * t30498 - 0.17563392970889009434e0 * t30500 - 0.2600466522016280569e0 * t6583 * t5109 * t32485 * t2573 - 0.20803732176130244552e1 * t30535 - 0.65854491829355115984e0 * t2122 * t7337 * t32679 - 0.76830240467580968651e0 * t30559;
    (t34410,)
}
