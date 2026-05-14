//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1191/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1191<F: Float>(t2035: F, t2735: F, t6979: F, t111834: F, t111841: F, t112107: F, t112127: F, t112153: F, t112156: F, t112159: F, t1472: F, t14872: F, t19101: F, t19107: F, t19135: F, t2413: F, t25070: F, t28598: F, t4093: F, t4104: F, t6035: F, t6976: F, t70683: F, t98527: F) -> (F,) {
    let t112163 = t2035 * t6979 * t2735;
    let t112183 = 0.22653425206514361674e0 * t1472 * t112153 - 0.16111841180489911311e0 * t112156 * t111834 + 0.16111841180489911311e0 * t112159 * t111841 - 0.10947790369858991997e1 * t19107 * t112163 + 0.45306850413028723348e0 * t70683 * t4093 * t6976 + 0.54738951849294959987e0 * t19135 * t112163 - 0.45306850413028723348e0 * t14872 * t4093 * t6976 + 0.21895580739717983994e1 * t19101 * t112107 + 0.24163653553615319118e1 * t4104 * t112127 + 0.22226000364197530866e-1 * t98527 - 0.33339000546296296297e-1 * t25070 * t6035 * t28598 * t2413;
    (t112183,)
}
