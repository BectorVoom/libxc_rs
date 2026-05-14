//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1445/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1445<F: Float>(t18908: F, t18916: F, t23708: F, t23711: F, t23715: F, t2449: F, t2881: F, t2995: F, t32093: F, t32097: F, t32102: F, t32103: F, t32121: F, t32122: F, t32128: F, t32144: F, t32204: F, t32216: F, t32270: F, t3250: F, t34834: F, t34851: F, t34855: F, t34871: F, t34877: F, t34882: F, t34897: F, t34909: F, t354: F) -> (F,) {
    let t34915 = 3.0 * t2995 * t2881 - t23708 + t32093 - t32097 + 3.0 * t2449 * t3250 + t23711 + t18908 + t354 * (t32102 + t32103 + t32122 + t32128 + t32144 + t32204 + t32216 + t32270 + t34834 + t34851 + t34855 + t34871 + t34877 + t34882 + t34897 + t34909) + t32121 + t23715 + t18916;
    (t34915,)
}
