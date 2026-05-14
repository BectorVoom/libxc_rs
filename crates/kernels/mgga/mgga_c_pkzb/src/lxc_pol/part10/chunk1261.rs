//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1261/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1261<F: Float>(t1719: F, t3460: F, t1044: F, t5389: F, t164: F, t1717: F, t1721: F, t1753: F, t183: F, t24046: F, t24316: F, t24435: F, t24715: F, t2682: F, t2693: F, t588: F, t600: F, t6865: F, t6881: F, t6898: F, t6903: F, t7084: F, t9019: F, t9056: F, t9067: F) -> (F, F) {
    let t24768 = t3460 * t1719;
    let t24792 = t5389 * t1044;
    let t24801 = -0.65854491829355115987e0 * t2693 * t24316 + 0.13170898365871023197e1 * t1717 * t24768 * t1721 - 0.13170898365871023197e1 * t588 * t9019 * t600 * t164 - 0.65854491829355115987e0 * t588 * t3460 * t1753 * t164 - 0.13170898365871023197e1 * t588 * t1044 * t7084 * t164 + 0.52683593463484092788e1 * t9056 * t6865 - 0.13170898365871023197e1 * t9067 * t6881 - 0.65854491829355115987e0 * t588 * t183 * t24715 * t164 - 0.79025390195226139182e1 * t24792 * t6898 + 0.79025390195226139182e1 * t9056 * t6903 - 0.13170898365871023197e1 * t2693 * t24046 + 0.15805078039045227836e2 * t2682 * t24435;
    (t24768, t24801)
}
