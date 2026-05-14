//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1047/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1047<F: Float>(t163: F, t164: F, t169: F, t171: F, t18035: F, t18053: F, t18067: F, t18072: F, t18077: F, t18079: F, t18089: F, t18091: F, t26417: F, t26419: F, t26432: F, t26437: F, t26439: F, t33426: F, t33431: F, t42252: F, t42265: F, t42272: F, t47672: F, t48423: F, t48436: F, t48621: F, t48622: F, t48625: F, t48626: F, t48629: F, t48630: F, t48632: F, t48634: F, t48638: F, t48640: F, t48642: F, t48645: F, t48646: F, t48648: F, t48651: F, t48656: F, t48657: F, t48659: F, t48663: F, t48667: F, t48669: F, t48671: F, t48674: F, t48678: F, t48679: F, t48681: F, t48682: F, t48686: F, t48689: F, t48694: F) -> (F,) {
    let t48706 = 0.75612977335538682803e0 * t26417 - 0.47461239486605618761e-3 * t26419 + 0.12602162889256447134e0 * t42252 - t18035 + 0.35124419763413520009e0 * t26432 - 0.12602162889256447134e0 * t26437 + 0.37806488667769341401e0 * t26439 - t18053 - t18067 - 0.12602162889256447134e0 * t42265 - t18072 + t18077 - t18079 + 0.35922702030763827281e-1 * t42272 + t18089 + t18091 - 0.189032443338846707e0 * t33426 + 0.37806488667769341401e0 * t33431 - 0.53884053046145740922e-2 * t169 * t171 * (t48681 + t48682 + t48648 + t48638 + t48671 + t47672 + t48674 + t48694 + t48621 + t48622 + t48625 + t48689 + t48663 + t48659 + t48423 + t48651 + t48686 + t48640 + t48642 + t48629 + t48630 + t48632 + t48667 + t48669 + t48678 + t48679 + t48656 + t48657 + t48645 + t48646 + t48626 + t48634) * t163 - 0.31505407223141117834e-1 * t48436 * t164;
    (t48706,)
}
